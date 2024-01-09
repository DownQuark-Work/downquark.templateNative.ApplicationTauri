import {LitElement, css, html} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement('layout-zone')
export class LayoutZone extends LitElement {
  // Define scoped styles right with your component, in plain CSS
  static styles = css`
    :host {
      color: floralwhite,
    }
  `;

  // Declare reactive properties
  @property()
  desc?: string = `Can be thought of as a "Page" or "View". Only one _Zone_ can be rendered at a time.
  The _Zone_ level is mainly for configuration. Titles, notes, metadata, custom event and hook values should
  all be set at this level.

  However, the _Zone_ is also the only level you are able to mount an \`implementation\`.
  Note that the layout defined in the \`implementation\` will take priority over what is defined in _Zone_ configuration.
  `;

  // Render the UI as a function of component state
  render() {
    return html`<p>ZONE:, ${this.desc}!</p>`;
  }
}

declare global { interface HTMLElementTagNameMap
  { 'layout-zone': LayoutZone } }