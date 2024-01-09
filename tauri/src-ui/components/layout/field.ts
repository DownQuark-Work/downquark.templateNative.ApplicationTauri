import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement('layout-field')
export class LayoutField extends LitElement {
  // Define scoped styles right with your component, in plain CSS
  static styles = css`
    :host {
      color: floralwhite,
    }
  `;

  // Declare reactive properties
  @property()
  desc?: string = `Concerned primarily with user interaction.
    The field is the bridge between the code and the human element.`;

  // Render the UI as a function of component state
  render() {
    return html`<p>FIELD:, ${this.desc}!</p>`;
  }
}

declare global { interface HTMLElementTagNameMap
  { 'layout-field': LayoutField; }}