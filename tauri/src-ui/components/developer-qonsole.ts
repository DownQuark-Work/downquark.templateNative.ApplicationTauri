import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';

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
  `;

  // Declare reactive properties
  @property()
  name?: string = 'I am the Developer Qonsole';

  // Render the UI as a function of component state
  render() {
    return html`
      <p>Hello, ${this.name}!</p>
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