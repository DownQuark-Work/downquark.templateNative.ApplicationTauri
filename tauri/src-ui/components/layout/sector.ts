import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement('layout-sector')
export class LayoutSector extends LitElement {
  // Define scoped styles right with your component, in plain CSS
  static styles = css`
    :host {
      color: floralwhite,
    }
  `;

  // Declare reactive properties
  @property()
  desc?: string = `The purpose of Sectors are mainly for organization. They were originally designed to allow related
    items to be placed alongside each other.
    A natural _Separation of Concerns_ occurs with this as well. Which is why we recommend registering all enclosed
    _executables_ and _widgets_ within these components`;

  // Render the UI as a function of component state
  render() {
    return html`<p>SECTOR:, ${this.desc}!</p>`;
  }
}

declare global { interface HTMLElementTagNameMap
  { 'layout-sector': LayoutSector } }