const api = typeof browser !== 'undefined' ? browser : chrome;

const NATIVE_HOST_NAME = 'com.myvault.app'; 
let nativePort = api.runtime.connectNative(NATIVE_HOST_NAME);

let lastRequestingTabId = null;
let lastRequestingFrameId = null; 
let isLoadedSent = false;

// ==========================================
// 1. LISTEN TO RUST -> ROUTE BACK TO BROWSER
// ==========================================
nativePort.onMessage.addListener((msg) => {
  console.log("Received from Rust:", msg);

  if (lastRequestingTabId !== null) {
    // Send strictly to the specific Tab AND Frame that requested it!
    api.tabs.sendMessage(lastRequestingTabId, msg, { frameId: lastRequestingFrameId }).catch((err) => {
      console.warn(`Tab ${lastRequestingTabId} frame ${lastRequestingFrameId} not ready:`, err.message);
    });
  } else {
    sendToActiveTab(msg);
  }
});

nativePort.onDisconnect.addListener(() => {
  console.error("Disconnected from Rust backend:", api.runtime.lastError?.message);
});



// ==========================================
// 2. LISTEN TO BROWSER -> ROUTE TO RUST
// ==========================================
api.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (!sender.tab || !sender.tab.id) return;
  
  const tabIdKey = `tab_data_${sender.tab.id}`;

  if (msg.action === 'SAVE_TAB_DATA') {
    // chrome.storage.session keeps data in memory. It is cleared when the browser closes.
    chrome.storage.session.set({ [tabIdKey]: request.payload }, () => {
      sendResponse({ success: true });
    });
    return true; // Indicates async response
  }

  if (msg.action === 'GET_TAB_DATA') {
    chrome.storage.session.get([tabIdKey], (result) => {
      sendResponse(result[tabIdKey] || {});
    });
    return true; // Indicates async response
  }

  if (msg.action === 'CLEAR_TAB_DATA') {
    chrome.storage.session.remove(tabIdKey);
    sendResponse({ success: true });
    return true;
  }

  // Capture BOTH Tab ID and Frame ID to prevent iframe broadcasting
  if (sender && sender.tab && sender.tab.id) {
    lastRequestingTabId = sender.tab.id;
    lastRequestingFrameId = sender.frameId !== undefined ? sender.frameId : 0; 
  }

  // Handle the one-time extension initialization globally
  if (msg.action === "ext_loaded") {
    if (!isLoadedSent) {
      console.log(`Forwarding ext_loadMSG to Rust`);
      nativePort.postMessage(msg);
      isLoadedSent = true;
    }
    return; // Stop here! Do not forward duplicate iframe "ext_loaded" messages
  }

  console.log(msg);
  nativePort.postMessage(msg);
});

// ==========================================
// HELPER FUNCTIONS
// ==========================================
function sendToActiveTab(msg) {
  api.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    if (tabs.length > 0) {
      api.tabs.sendMessage(tabs[0].id, msg, { frameId: 0 }).catch(() => {});
    }
  });
}

api.tabs.onRemoved.addListener((tabId) => {
  chrome.storage.session.remove(`tab_data_${tabId}`);
});