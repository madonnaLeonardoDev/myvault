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

let lastFilledCredentials = { username: null, password: null };

function fillCredentials(username, password) {
  const setInputValue = (inputElement, val) => {
    if (!inputElement || val == null) return;

    inputElement.focus();

    const nativeSetter = Object.getOwnPropertyDescriptor(
      window.HTMLInputElement.prototype,
      'value'
    )?.set;

    if (nativeSetter) {
      nativeSetter.call(inputElement, val);
    } else {
      inputElement.value = val;
    }

    inputElement.dispatchEvent(new InputEvent('input', {
      bubbles: true,
      cancelable: true,
      inputType: 'insertText',
      data: val
    }));
    
    inputElement.dispatchEvent(new Event('change', { bubbles: true }));
  };

  const findInput = (selector) => {
    let el = document.querySelector(selector);
    if (el) return el;

    const allElements = document.querySelectorAll('*');
    for (const host of allElements) {
      if (host.shadowRoot) {
        el = host.shadowRoot.querySelector(selector);
        if (el) return el;
      }
    }
    return null;
  };

  const pwInput = findInput('input[type="password"]');
  const userInput = findInput(
    'input[type="text"], input[type="email"], input[name*="user"], input[name*="login"], input[autocomplete="username"]'
  );

  if (userInput && username) {
    setInputValue(userInput, username);
  }

  if (pwInput && password) {
    setInputValue(pwInput, password);
  }

  // Record what was filled so we can suppress the save prompt if nothing changes
  lastFilledCredentials = {
    username: username || "",
    password: password || ""
  };
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

sendToRust("ext_loaded", "", "", "");


function isLoginField(input) {
  if (input.type === "password") return true;

  if (input.type === "text" || input.type === "email") {
    const isSearch = /search|query|q/i.test(input.name + input.id + input.className + input.placeholder);
    if (isSearch || input.type === "search" || input.readOnly) return false;

    const ac = input.autocomplete.toLowerCase();
    if (ac === "username" || ac === "email" || ac === "webauthn") return true;

    const attrString = (input.name + input.id + input.placeholder).toLowerCase();
    const hasLoginKeywords = /user|login|email|account|sign-in|signin|id/i.test(attrString);
    if (hasLoginKeywords) return true;

    const form = input.closest("form");
    if (form && form.querySelector('input[type="password"]')) {
      return true;
    }
  }

  return false;
}

document.addEventListener("focusin", (event) => {
  const target = /** @type {HTMLElement} */ (event.target);
  if (!(target instanceof HTMLInputElement)) return;

  if (isLoginField(target)) {
    sendToRust("field_focused", window.location.hostname, "", "");
  }
}, true);

function getLoginInput() {
  const pwInput = document.querySelector('input[type="password"]');
  const userInput = document.querySelector(
    'input[type="text"]:not([type="search"]), input[type="email"], input[autocomplete="username"]'
  );

  return {
    username: (userInput && userInput.value) ? userInput.value : null,
    password: (pwInput && pwInput.value) ? pwInput.value : null
  };
}

function submitLoginListener(onSubmit) {
  function handleCapture(targetElement) {
    const container = targetElement.closest('form') || targetElement.closest('div') || document.body;
    const pwInput = container.querySelector('input[type="password"]');
    if (!pwInput || !pwInput.value) return;

    const userInput = container.querySelector(
      'input[type="text"]:not([type="search"]), input[type="email"], input[autocomplete="username"]'
    );

    onSubmit({
      username: (userInput && userInput.value) ? userInput.value : null,
      password: pwInput.value
    });
  }

  document.addEventListener('submit', (e) => {
    handleCapture(e.target);
  }, true);

  document.addEventListener('click', (e) => {
    const btn = e.target.closest('button, input[type="submit"], input[type="button"], [role="button"]');
    if (!btn) return;

    if (btn.closest('form')) return;

    const btnText = (btn.textContent || btn.value || '').toLowerCase();
    const isLoginBtn = /log\s*in|sign\s*in|submit|continue|next/i.test(btnText);

    if (btn.type === 'submit' || isLoginBtn) {
      handleCapture(btn);
    }
  }, true);
}

submitLoginListener((credentials) => {
    if (
      lastFilledCredentials.username !== null &&
      credentials.username === lastFilledCredentials.username &&
      credentials.password === lastFilledCredentials.password
    ) {
      return;
    }

    const { hostElement, shadowRoot } = build_popup(password_add("save password?"));
    const decline = shadowRoot.querySelector('.decline');
    const accept = shadowRoot.querySelector('.accept');

    accept.addEventListener('click', () => {
      const {username, password} = credentials;
      if (username === null || password === null) {
        return;
      }
      sendToRust("save_pw", window.location.hostname, username, password);
      closePopup();
    });

    decline.addEventListener('click', () => {
      closePopup();
    });
});

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
    const now = Date.now();
    if (now - lastMatchTime < 15000) return;
    lastMatchTime = now;

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
        fillCredentials(selectedAccount.username, selectedAccount.password);
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