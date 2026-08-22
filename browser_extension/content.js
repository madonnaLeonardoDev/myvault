const fontUrl = (typeof browser !== 'undefined' ? browser : chrome).runtime.getURL('fonts/JetBrainsMono-Regular.woff2');

function request_pw(txt) { return`
    <style>
      @font-face {
        font-family: 'JetBrains Mono';
        src: url('${fontUrl}') format('woff2');
        font-weight: 400;
        font-style: normal;
      }

      :host {
        color-scheme: light dark;

        /* Light Theme */
        --bg-color: #ffffff;
        --primary-color: #121212;
        --secondary-color: #6c7086;
        --button-color: #4f46e5;
        --hover-color1: #4338ca;
        --hover-color2: #515464;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          /* Dark Theme */
          --bg-color: #121212;
          --primary-color: #e2e8f0;
          --secondary-color: #94a3b8;
          --button-color: #6366f1;
          --hover-color1: #818cf8;
          --hover-color2: #64748b;
        }
      }

      .myvault-container {
        display: flex;
        flex-direction: column;
        border-radius: 0px;
        margin: 0;
        padding: 2px 4px 2px 4px;
        min-width: 200px;
        width: fit-content;
        height: fit-content;
        background-color: var(--bg-color);
        border: 0;
        font-family: 'JetBrains Mono', monospace;
        font-size: 13px;
        box-sizing: border-box;
        border-radius: 3px;
        box-shadow: 0px 4px 16px -10px var(--primary-color);
      }

      .myvault-container * {
        box-sizing: border-box;
      }

      .title {
        align-self: flex-start;
        color: var(--secondary-color);
        text-decoration: none;
        margin: 5px 0 0 5px;
      }

      .input {
        border-radius: 3px;
        padding: 3px;
        margin: 8px 5px 8px 5px;
        background-color: var(--secondary-color);
        color: white;
        outline: none;
        border: none;
        font-family: 'JetBrains Mono', monospace;
        -webkit-text-security: disc;
      }

      .input:focus {
        outline: none;
      }

      .buttonsection {
        display: flex;
        margin: auto 10px 5px 10px;
        align-self: center;
        width: 100%;
      }

      .buttonsection button {
        cursor: pointer;
        border-radius: 5px;
        border: none;
        width: 50%;
        font-weight: bold;
        padding: 6px;
        margin: 0 5px 0 5px;
        color: white;
        font-family: 'JetBrains Mono', monospace;
      }

      .decline {
        background-color: var(--secondary-color);
      }

      .decline:hover {
        background-color: var(--hover-color2);
      }

      .accept {
        background-color: var(--button-color);
      }

      .accept:hover {
        background-color: var(--hover-color1);
      }
    </style>

    <div class="myvault-container">
      <a href="https://github.com/madonnaLeonardoDev/myvault" class="title">myvault</a>
      <input type="password" placeholder="${txt}" class="input">
      <div class="buttonsection">
        <button class="decline">close</button>
        <button class="accept">confirm</button>
      </div>
    </div>
  `;}

function password_add(txt) { return `
    <style>
      @font-face {
        font-family: 'JetBrains Mono';
        src: url('${fontUrl}') format('woff2');
        font-weight: 400;
        font-style: normal;
      }

      :host {
        color-scheme: light dark;

        /* Light Theme */
        --bg-color: #ffffff;
        --primary-color: #121212;
        --secondary-color: #6c7086;
        --button-color: #4f46e5;
        --hover-color1: #4338ca;
        --hover-color2: #515464;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          /* Dark Theme */
          --bg-color: #121212;
          --primary-color: #e2e8f0;
          --secondary-color: #94a3b8;
          --button-color: #6366f1;
          --hover-color1: #818cf8;
          --hover-color2: #64748b;
        }
      }

      .myvault-container {
        display: flex;
        flex-direction: column;
        border-radius: 0px;
        margin: 0;
        padding: 2px 4px 2px 4px;
        min-width: 200px;
        width: fit-content;
        height: fit-content;
        background-color: var(--bg-color);
        border: 0;
        font-family: 'JetBrains Mono', monospace;
        font-size: 13px;
        box-sizing: border-box;
        border-radius: 3px;
        box-shadow: 0px 4px 16px -10px var(--primary-color);
      }

      .myvault-container * {
        box-sizing: border-box;
      }

      .title {
        align-self: flex-start;
        color: var(--secondary-color);
        text-decoration: none;
        margin: 5px 0 0 5px;
      }

      .buttonsection {
        display: flex;
        margin: auto 10px 5px 10px;
        align-self: center;
        width: 100%;
      }

      .buttonsection button {
        cursor: pointer;
        border-radius: 5px;
        border: none;
        width: 50%;
        font-weight: bold;
        padding: 6px;
        margin: 0 5px 0 5px;
        color: white;
        font-family: 'JetBrains Mono', monospace;
      }

      .decline {
        background-color: var(--secondary-color);
      }

      .decline:hover {
        background-color: var(--hover-color2);
      }

      .accept {
        background-color: var(--button-color);
      }

      .accept:hover {
        background-color: var(--hover-color1);
      }
    </style>

    <div class="myvault-container">
      <a href="https://github.com/madonnaLeonardoDev/myvault" class="title">myvault</a>
      <p style="color: white; margin: 8px; text-align: center;">${txt}</p>
      <div class="buttonsection">
        <button class="decline">close</button>
        <button class="accept">confirm</button>
      </div>
    </div>
  `;}

function notif(txt){
  return `
    <style>
      @font-face {
        font-family: 'JetBrains Mono';
        src: url('${fontUrl}') format('woff2');
        font-weight: 400;
        font-style: normal;
      }

      :host {
        color-scheme: light dark;

        /* Light Theme */
        --bg-color: #ffffff;
        --primary-color: #121212;
        --secondary-color: #6c7086;
        --button-color: #4f46e5;
        --hover-color1: #4338ca;
        --hover-color2: #515464;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          /* Dark Theme */
          --bg-color: #121212;
          --primary-color: #e2e8f0;
          --secondary-color: #94a3b8;
          --button-color: #6366f1;
          --hover-color1: #818cf8;
          --hover-color2: #64748b;
        }
      }

      @keyframes fadeToZero {
        from { opacity: 1; }
        to { opacity: 0; }   
      }

      .myvault-container {
        display: flex;
        flex-direction: column;
        border-radius: 0px;
        margin: 0;
        padding: 2px 4px 2px 4px;
        min-width: 200px;
        width: fit-content;
        height: fit-content;
        background-color: var(--bg-color);
        border: 0;
        font-family: 'JetBrains Mono', monospace;
        font-size: 13px;
        box-sizing: border-box;
        border-radius: 3px;
        box-shadow: 0px 4px 16px -10px var(--primary-color);
        animation: fadeToZero 5s linear forwards;
      }

      .myvault-container:hover {
        animation: none;
        opacity: 1;
      }

      .myvault-container * {
        box-sizing: border-box;
      }

      .title {
        align-self: flex-start;
        color: var(--secondary-color);
        text-decoration: none;
        margin: 5px 0 0 5px;
      }

      .message {
        padding: 3px;
        background-color: var(--secondary-color);
        height: fit-content;
        border-radius: 3px;
        margin: 8px 5px 8px 5px;
        color: white;
      }

      .message p {
        margin: 0;
        padding: 0;
      }

    </style>

    <div class="myvault-container">
      <a href="https://github.com/madonnaLeonardoDev/myvault" class="title">myvault</a>
      <div class="message">
        <p>${txt}</p>
      </div>
    </div>
  `;}

function build_popup(html) {
  closePopup()
  const hostElement = document.createElement('div');
  hostElement.id = 'myvault-root';

  hostElement.setAttribute('popover', 'manual');

  Object.assign(hostElement.style, {
    position: 'fixed',
    top: '16px',
    right: '16px',
    bottom: 'auto',
    left: 'auto',
    margin: '0',
    padding: '0',
    border: 'none',
    background: 'transparent',
    zIndex: '2147483647',
    pointerEvents: 'auto'
  });

  const shadowRoot = hostElement.attachShadow({ mode: 'open' });
  shadowRoot.innerHTML = html;

  document.body.appendChild(hostElement);

  if (typeof hostElement.showPopover === 'function') {
    try { hostElement.showPopover(); } catch (e) {}
  }

  return { hostElement, shadowRoot };
}

function closePopup() {
  // Use querySelectorAll to find ALL instances just in case duplicates spawned
  const existings = document.querySelectorAll('#myvault-root');
  existings.forEach((existing) => {
    if (typeof existing.hidePopover === 'function') {
      try { existing.hidePopover(); } catch (e) {}
    }
    existing.remove();
  });
}
function escapeHtml(str) {
  const div = document.createElement('div');
  div.textContent = str == null ? '' : String(str);
  return div.innerHTML;
}

function sendToRust(action, website, username, password) {
  const packet = {
    action: action || "",
    website: website || "",
    username: username || "",
    password: password || ""
  };
  (typeof browser !== 'undefined' ? browser : chrome).runtime.sendMessage(packet);
}

//SUBMIT LISTENER 
let lastFill;

function initializeLoginInterceptor() {
  let pendingUsername = '';

  function extractCredentials(container) {
    // 1. Grab every input inside the form (or document)
    const allInputs = Array.from(container.querySelectorAll('input'));
    
    // 2. Filter them using your exact autofill logic!
    const userFields = allInputs.filter(el => isUsernameField(el));
    const passFields = allInputs.filter(el => isPasswordFieldOnly(el));

    // 3. SPA FIX: Find the first field that actually has text in it. 
    // This bypasses hidden modal inputs (like the Pinterest "Sign Up" vs "Log In" issue).
    const userField = userFields.find(input => input.value.trim().length > 0) || userFields[0];
    const passField = passFields.find(input => input.value.length > 0) || passFields[0];

    return {
      username: userField ? userField.value.trim() : '',
      password: passField ? passField.value : ''
    };
  }

  function handleInteraction(e) {
    const isSubmitEvent = e.type === 'submit';
    // Broadened to catch divs/spans used as buttons (very common in React apps)
    const isButtonClick = e.type === 'click' && (e.target.closest('button, [role="button"], input[type="submit"]') !== null);
    const isEnterKey = e.type === 'keydown' && e.key === 'Enter' && e.target.tagName === 'INPUT';

    if (!isSubmitEvent && !isButtonClick && !isEnterKey) return;

    const container = e.target.closest('form') || document;
    const { username, password } = extractCredentials(container);

    if (password) {
      const finalUsername = username || pendingUsername;
      
      // FIX: Safely compare object properties instead of object references!
      // (lastFill === { ... }) will always be false in JavaScript.
      if (lastFill && lastFill.username === finalUsername && lastFill.password === password) {
        console.log("fah: Credentials match last fill, skipping.");
        return;
      }
      
      console.log('Login attempt detected:', { username: finalUsername, password: '***' });

      const { hostElement, shadowRoot } = build_popup(password_add("Save Password?"));
    const accept = shadowRoot.querySelector('.accept');
    const decline = shadowRoot.querySelector('.decline');

    accept.addEventListener('click', () => {
        sendToRust("save_pw", window.location.hostname, finalUsername, password);
        closePopup();
    });

    decline.addEventListener('click', () => {
      closePopup();
    });
      
      // Update lastFill so we don't spam the Rust backend if the user clicks twice
      lastFill = { username: finalUsername, password: password };
      pendingUsername = ''; // Reset state after a password submit

    } else if (username && (isButtonClick || isEnterKey || isSubmitEvent)) {
      // FIX: Added parentheses around the events so it requires a username to be present
      pendingUsername = username;
    }
  }

  // Use { capture: true } to intercept the event on its way DOWN the DOM tree.
  document.addEventListener('submit', handleInteraction, true);
  document.addEventListener('click', handleInteraction, true);
  document.addEventListener('keydown', handleInteraction, true);
}

initializeLoginInterceptor()



//DOM FILL PASSWORD AND FILL USERNAME

 
function setNativeValue(el, value) {
  const proto = Object.getPrototypeOf(el);
  const descriptor = Object.getOwnPropertyDescriptor(proto, 'value');
  const nativeSetter = descriptor && descriptor.set;
  if (nativeSetter) {
    nativeSetter.call(el, value);
  } else {
    el.value = value;
  }
  el.dispatchEvent(new Event('input', { bubbles: true }));
  el.dispatchEvent(new Event('change', { bubbles: true }));
}
 
function fillUsernameField(root, value) {
  const inputs = root.querySelectorAll('input');
  for (const el of inputs) {
    if (isUsernameField(el)) {
      setNativeValue(el, value);
      return true;
    }
  }
  return false;
}
 
function fillPasswordField(root, value) {
  const inputs = root.querySelectorAll('input');
  for (const el of inputs) {
    if (isPasswordFieldOnly(el)) {
      setNativeValue(el, value);
      return true;
    }
  }
  return false;
}

 
function fillAll(root, username, password) {
  fillUsernameField(root, username);
 
  fillPasswordField(root, password);

  lastFill = {username: username, password: password};
}

//DOM LISTENERS FOCUS LOGIN FIELD
function isPasswordFieldOnly(el) {
  if (!el || el.tagName !== 'INPUT') return false;
  if (el.type === 'password') return true;
  const autocomplete = (el.getAttribute('autocomplete') || '').toLowerCase();
  if (autocomplete === 'current-password' || autocomplete === 'new-password') return true;
  if (el.name === 'Passwd') return true;
  return false;
}
 
function isUsernameField(el) {
  if (!el || el.tagName !== 'INPUT') return false;
  if (isPasswordFieldOnly(el)) return false;
  const autocomplete = (el.getAttribute('autocomplete') || '').toLowerCase();
  if (autocomplete === 'username' || autocomplete === 'email') return true;
  if (el.id === 'identifierId') return true;
  const usernameTypes = ['text', 'email', 'tel', ''];
  if (usernameTypes.includes(el.type)) {
    const haystack = (
      (el.name || '') + ' ' +
      (el.id || '') + ' ' +
      (el.getAttribute('placeholder') || '')
    ).toLowerCase();
    if (/user|username|login|identifier|email|e-mail/.test(haystack)) return true;
  }
  return false;
}
 
function isLoginField(el) {
  return isUsernameField(el) || isPasswordFieldOnly(el);
}

document.addEventListener('focus', (e) => {
  if (isLoginField(e.target)) {
    console.log('Login field focused:', e.target);
    sendToRust("field_focused", window.location.hostname)
  }
}, true);

document.addEventListener('focus', (e) => {
  if (isLoginField(e.target)) {
    console.log('Login field focused:', e.target);
    sendToRust("field_focused", window.location.hostname)
  }
}, true);
//START MESSAGE
sendToRust("ext_loaded", "", "", "");
//DAEMON LISTENER
const api = typeof browser !== 'undefined' ? browser : chrome;
let lastMatchTime = 0;

api.runtime.onMessage.addListener((msg) => {
  console.log("Received from Rust:", msg);
  
  if (msg.status === 'ask_pw') {
    const { hostElement, shadowRoot } = build_popup(request_pw(msg.message));
    const input = shadowRoot.querySelector('.input');
    const accept = shadowRoot.querySelector('.accept');
    const decline = shadowRoot.querySelector('.decline');

    accept.addEventListener('click', () => {
      if (input.value !== '') {
        sendToRust('password', null, null, input.value);
        closePopup();
      }
    });

    decline.addEventListener('click', () => {
      closePopup();
    });
  }  
  
  if (msg.status === 'match_found') {
    const matches = typeof msg.message === 'string' ? JSON.parse(msg.message) : msg.message;

    const fill = `
    <style>
      @font-face {
        font-family: 'JetBrains Mono';
        src: url('${fontUrl}') format('woff2');
        font-weight: 400;
        font-style: normal;
      }

      :host {
        color-scheme: light dark;
        --bg-color: #ffffff;
        --primary-color: #121212;
        --secondary-color: #6c7086;
        --button-color: #4f46e5;
        --hover-color1: #4338ca;
        --hover-color2: #515464;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          --bg-color: #121212;
          --primary-color: #e2e8f0;
          --secondary-color: #94a3b8;
          --button-color: #6366f1;
          --hover-color1: #818cf8;
          --hover-color2: #64748b;
        }
      }

      .myvault-container {
        display: flex;
        flex-direction: column;
        border-radius: 0px;
        margin: 0;
        padding: 2px 4px 2px 4px;
        min-width: 200px;
        width: fit-content;
        height: fit-content;
        background-color: var(--bg-color);
        border: 0;
        font-family: 'JetBrains Mono', monospace;
        font-size: 13px;
        box-sizing: border-box;
        border-radius: 3px;
        box-shadow: 0px 4px 16px -10px var(--primary-color);
      }

      .myvault-container * {
        box-sizing: border-box;
      }

      .title {
        align-self: flex-start;
        color: var(--secondary-color);
        text-decoration: none;
        margin: 5px 0 0 5px;
      }

      .message {
        padding: 3px;
        background-color: var(--secondary-color);
        height: fit-content;
        border-radius: 3px;
        margin: 8px 5px 8px 5px;
        color: white;
      }

      .message p {
        margin: 0;
        padding: 0;
      }

      .buttonsection {
        display: flex;
        margin: auto 10px 5px 10px;
        align-self: center;
        width: 100%;
      }

      .buttonsection button {
        cursor: pointer;
        border-radius: 5px;
        border: none;
        width: 50%;
        font-weight: bold;
        padding: 6px;
        margin: 0 5px 0 5px;
        color: white;
        font-family: 'JetBrains Mono', monospace;
      }

      .decline {
        background-color: var(--secondary-color);
      }

      .decline:hover {
        background-color: var(--hover-color2);
      }

      .accept {
        background-color: var(--button-color);
      }

      .accept:hover {
        background-color: var(--hover-color1);
      }
    </style>

    <div class="myvault-container">
      <a href="https://github.com/madonnaLeonardoDev/myvault" class="title">myvault</a>
      <div class="message">
        <div class="matches-list">
          ${matches.map((item, index) => `
            <div class="match-item" data-index="${index}" style="cursor: pointer;">
              <p><strong>${escapeHtml(item.username)}</strong></p>
            </div>
          `).join('')}
        </div>
      </div>
      <div class="buttonsection">
        <button class="decline">close</button>
        <button class="accept">fill</button>
      </div>
    </div>
  `;

    const { hostElement, shadowRoot } = build_popup(fill);
    const listContainer = shadowRoot.querySelector('.matches-list');
    const accept = shadowRoot.querySelector('.accept');
    const decline = shadowRoot.querySelector('.decline');

    let selectedAccount = null;

    listContainer.addEventListener('click', (event) => {
      const target = /** @type {HTMLElement} */ (event.target);
      const clickedItem = target.closest('.match-item');
      if (!clickedItem) return;

      listContainer.querySelectorAll('.match-item').forEach((el) => {
        /** @type {HTMLElement} */ (el).style.color = '';
      });
      /** @type {HTMLElement} */ (clickedItem).style.color = 'black';

      const index = parseInt(clickedItem.getAttribute('data-index'), 10);
      selectedAccount = matches[index];
    });

    accept.addEventListener('click', () => {
      if (selectedAccount) {
        fillAll(document, selectedAccount.username, selectedAccount.password);
        closePopup();
      }
    });

    decline.addEventListener('click', () => {
      closePopup();
    });
  }

  if (msg.status === 'overwrite_save') {
    const { hostElement, shadowRoot } = build_popup(password_add("overwrite password?"));
    const decline = shadowRoot.querySelector('.decline');
    const accept = shadowRoot.querySelector('.accept');

    accept.addEventListener('click', () => {
      const website_usr_pw = typeof msg.message === 'string' ? JSON.parse(msg.message) : msg.message;
      if (website_usr_pw[0] === "" || website_usr_pw[1] === "" || website_usr_pw[2] === "") {
        return;
      }
      sendToRust("overwrite_pw", website_usr_pw[0], website_usr_pw[1], website_usr_pw[2]);
      closePopup();
    });

    decline.addEventListener('click', () => {
      closePopup();
    });
  }

  if (msg.status === 'ok' || msg.status === 'error') {
    const { hostElement, shadowRoot } = build_popup(notif(msg.message));
    const popup_message = shadowRoot.querySelector('p');
    
    if (msg.status === 'error') {
      popup_message.style.backgroundColor = '#db2a2a';
    }

    const decline = shadowRoot.querySelector('.decline');
    const accept = shadowRoot.querySelector('.accept');
    if (decline) decline.addEventListener('click', closePopup);
    if (accept) accept.addEventListener('click', closePopup);

    // Automatically remove from DOM when the fade-out animation finishes
    const container = shadowRoot.querySelector('.myvault-container');
    if (container) {
      container.addEventListener('animationend', (e) => {
        if (e.animationName === 'fadeToZero') {
          closePopup();
        }
      });
    }
  }
});