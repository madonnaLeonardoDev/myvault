const api = typeof browser !== 'undefined' ? browser : chrome;

const NATIVE_HOST_NAME = 'com.myvault.app'; 
let nativePort = api.runtime.connectNative(NATIVE_HOST_NAME);


let lastRequestingTabId = null;
let pendingMessageFromRust = null; 

nativePort.onMessage.addListener((msg) => {
  console.log("Received from Rust:", msg);

  pendingMessageFromRust = msg; 

  if (lastRequestingTabId !== null) {
    api.tabs.sendMessage(lastRequestingTabId, msg).catch(() => {
      // Fallback: If the original tab was just closed, try the current active tab
      sendToActiveTab(msg);
    });
  } else {
    sendToActiveTab(msg);
  }
});

nativePort.onDisconnect.addListener(() => {
  console.error("Disconnected from Rust backend:", api.runtime.lastError?.message);
});


api.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  

  if (sender && sender.tab && sender.tab.id) {
    lastRequestingTabId = sender.tab.id;

    if (pendingMessageFromRust) {
      api.tabs.sendMessage(sender.tab.id, pendingMessageFromRust).catch(() => {});
      pendingMessageFromRust = null;
    }
  }

  nativePort.postMessage(msg);
});

function sendToActiveTab(msg) {
  api.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    if (tabs.length > 0) {
      api.tabs.sendMessage(tabs[0].id, msg).catch(() => {});
    }
  });
}