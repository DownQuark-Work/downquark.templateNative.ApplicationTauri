import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement('layout-region')
export class LayoutRegion extends LitElement {
  // Define scoped styles right with your component, in plain CSS
  static styles = css`
    :host {
      color: floralwhite,
    }
  `;

  // Declare reactive properties
  @property()
  desc?: string = `Most likely to find widgets at this level. Listening for hooks and dispatching events
    to both the sector and field layouts`;

  // Render the UI as a function of component state
  render() {
    return html`<p>REGION:, ${this.desc}!</p>`;
  }
}

declare global { interface HTMLElementTagNameMap
{ 'layout-region': LayoutRegion } }