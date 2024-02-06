import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import { invoke } from "@tauri-apps/api/core";

import './layout/field';
import './layout/region';
import './layout/sector';
import './layout/zone';

@customElement('developer-qonsole')
export class DeveloperQonsole extends LitElement {
  // Define scoped styles right with your component, in plain CSS
  static styles = css`
    :host {
      color: teal;
    }
    li{ color: blanchedalmond; }
    .devqon.state-trigger {
      color: darkcyan;
      display:flex;
      justify-content: space-around;
    }
    button {
  padding: 10px 20px;
  text-transform: uppercase;
  border-radius: 8px;
  font-weight: 500;
  color: teal;
  text-shadow: none;
  background: transparent;
  cursor: pointer;
  box-shadow: transparent;
  border: 1px solid #ffffff80;
  transition: 0.5s ease;
  user-select: none;

  :hover,
  :focus {
    color: #ffffff;
    background: teal;
    border: 1px solid teal;
    text-shadow: 0 0 5px #ffffff, 0 0 10px #ffffff, 0 0 20px #ffffff;
    box-shadow: 0 0 5px teal, 0 0 10px teal, 0 0 15px teal,
      0 0 3rem teal;
  }
}
  `;

  // Declare reactive properties
  @property()
  name?: string = 'I am the Developer Qonsole';

  _rustStore(triggerKey:string){
    invoke(`cmd_state_trigger_${triggerKey.toLowerCase()}`,{ number: 1234, })
    .then((res:any) => console.log(`created: ${res.created}, SESSION_ID: ${res.session_id}`)).catch((e) => console.error(e))
  }

  // update_current_vision_setting :: single field overwrite
  updateCurrentVesionSetting(){ invoke('update_current_vision_setting',{ updated: ''+Date.now(), })
    .then((res:any) => console.log('current vision settings: ', res)).catch((e) => console.error(e))
  }

  // update_history_state :: overwrite adds to list
  updateHistoryState(){ invoke('track_navigation',{ at: Date.now(), page: btoa(''+Date.now()), })
    .then((res:any) => console.log('current history state: ', res)).catch((e) => console.error(e))
  }

  // Render the UI as a function of component state
  render() {
    return html`
      <p>Hello, ${this.name}!</p>
      <h5>Current State: coming soon...</h5>
      <div class="[ devqon ][ state-trigger ]">
        <button @click="${()=>this._rustStore('US')}" type="button">access static state</button>
        <button @click="${this.updateCurrentVesionSetting}" type="button">dynamic state overwrite</button>
        <button @click="${this.updateHistoryState}" type="button">dynamic state list</button>
        <button type="button">plugin dynamic</button>
      </div>
      <layout-field>layout filed loading</layout-field>
      <layout-region>layout region loading</layout-region>
      <layout-sector>layout sector loading</layout-sector>
      <layout-zone>layout zone loading</layout-zone>
      
      <p>Below may come in handy:</p>
      <ul>
        <li>https://github.com/web-padawan/awesome-lit</li>
        <li> --- </li>
        <li>https://lit.dev/docs/templates/directives/#choose</li>
        <li>https://lit.dev/docs/templates/directives/#keyed</li>
        <li>https://lit.dev/docs/templates/directives/#live</li>
        <li>https://lit.dev/docs/templates/directives/#ref</li>
        <li>https://lit.dev/docs/templates/directives/#unsafesvg</li>
        <li>https://lit.dev/docs/templates/directives/#until</li>
        <li>https://lit.dev/docs/templates/directives/#asyncappend</li>
        <li>https://lit.dev/docs/templates/directives/#asyncreplace</li>
        <li>https://lit.dev/docs/composition/component-composition/#passing-data-across-the-tree</li>
        <li>https://lit.dev/docs/composition/controllers/
          <ul>
            <li>Reactive controllers allow you to build components by composing smaller pieces that aren't themselves components.</li>
          </ul>
        </li>
        <li>https://lit.dev/docs/data/context/
          <ul>
            <li>Context is a way of making data available to entire component subtrees without having to manually bind properties to every component</li>
          </ul>
        </li>
        <li>https://lit.dev/docs/data/task/
          <ul>
            <li>Task is a controller that takes an async task function and runs it either manually or automatically when its arguments change.</li>
          </ul>
        </li>
      </ul>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "developer-qonsole": DeveloperQonsole;
  }
}