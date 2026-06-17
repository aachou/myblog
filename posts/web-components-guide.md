+++
title = "Web Components Guide: Building Framework-Agnostic UI"
date = "2026-04-20"
tags = ["web-components", "javascript", "frontend"]
excerpt = "Web Components let you write reusable UI elements that work in any framework. This guide covers the three core technologies and practical patterns for real projects."
+++

Web Components are a browser-native way to create reusable custom elements. Unlike framework components, they work across React, Vue, Angular, and vanilla HTML without any adapter layer.

## The Three Pillars

Web Components consist of three browser APIs that work together:

1. **Custom Elements** 鈥?define new HTML tags
2. **Shadow DOM** 鈥?encapsulate styles and markup
3. **HTML Templates** 鈥?declare fragments that are not rendered until needed

```javascript

class MyCard extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
            <style>
                .card {
                    border: 1px solid #ccc;
                    border-radius: 8px;
                    padding: 16px;
                }
            </style>
            <div class="card">
                <slot name="title"></slot>
                <slot></slot>
            </div>
        `;
    }
}

customElements.define("my-card", MyCard);
```

Now `<my-card>` is available anywhere in your HTML.

## Lifecycle Callbacks

Custom Elements have four lifecycle methods:

| Callback | Trigger |
|---|---|
| `constructor()` | Element is created |
| `connectedCallback()` | Element is inserted into the DOM |
| `disconnectedCallback()` | Element is removed from the DOM |
| `attributeChangedCallback(name, old, new)` | Observed attribute changes |

Use `connectedCallback` for setup and `disconnectedCallback` for cleanup.

## Observed Attributes

To react to attribute changes, declare a static `observedAttributes` getter:


```javascript
class MyButton extends HTMLElement {
  static get observedAttributes() {
    return ["variant", "disabled"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue === newValue) return;
    this.render();
  }
}
```

This pattern lets you control your component declaratively through HTML attributes.

## Slots for Composition

Slots allow users to project content into your component from the outside.

```html
<my-card>
    <span slot="title">Hello World</span>
    <p>This content goes into the default slot.</p>
</my-card>

```
Named slots target specific insertion points, while the unnamed slot catches everything else.

## Styling and Shadow DOM

The Shadow DOM provides style encapsulation. Styles defined inside a shadow tree do not leak out, and external styles do not leak in.

```css
/* Inside shadow DOM */
:host {
  display: block;
  font-family: sans-serif;
}

:host([variant="primary"]) {
  background-color: blue;
  color: white;
}
```

Use `:host` to style the custom element itself, and `:host([attr])` to style based on attributes.

## Events in Web Components

Dispatch custom events to communicate with the outside world:

```javascript
class MyToggle extends HTMLElement {
    connectedCallback() {
        this.addEventListener("click", () => {
            this.dispatchEvent(new CustomEvent("toggle", {
                detail: { active: !this.active },
                bubbles: true,
                composed: true,
            }));
        });
    }
}

```
Setting `composed: true` lets the event cross shadow DOM boundaries.

## Framework Interop

Web Components work naturally in most frameworks. In React, you may need a small wrapper because React uses its own synthetic event system:

```jsx
function MyCardWrapper(props) {
  const ref = useRef();
  useEffect(() => {
    const el = ref.current;
    el.addEventListener("toggle", props.onToggle);
    return () => el.removeEventListener("toggle", props.onToggle);
  });
  return <my-card ref={ref} {...props} />;
}
```

Vue and Angular handle custom elements natively with almost no configuration.

## Conclusion

Web Components are not a framework replacement. They are a low-level primitive for building framework-agnostic UI pieces. For design systems, widget libraries, and components that need to outlive any particular framework, they are the right tool for the job.
