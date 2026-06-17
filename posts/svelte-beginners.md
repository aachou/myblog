+++
title = "Svelte for Beginners: Reactive Web Development Made Simple"
date = "2025-10-03"
tags = ["svelte", "frontend", "javascript"]
excerpt = "Svelte shifts the work from the browser to the compiler, resulting in fast, lean web applications. Start building with Svelte today."
+++

Svelte is a radical approach to building user interfaces. Unlike React or Vue, Svelte shifts the work from the browser to the compile step, producing highly optimized vanilla JavaScript.

## Why Svelte?

- No virtual DOM: direct DOM updates are faster
- Less boilerplate: write fewer lines of code
- Reactive by default: no hooks or dependency arrays
- Small bundle size: only the code you use ships
- Built-in state management: no external libraries needed

## Your First Component

```svelte

<script>
    let name = 'world';
</script>

<h1>Hello {name}!</h1>

<style>
    h1 {
        color: purple;
    }
</style>
```

## Reactivity

Svelte's reactivity is declarative:


```svelte
<script>
  let count = 0;

  function increment() {
    count += 1;
  }

  $: doubled = count * 2;
  $: console.log(`count is ${count}`);
</script>

<button on:click={increment}>
  Clicked {count} times
</button>
<p>Doubled: {doubled}</p>
```

## Props and Component Composition

```svelte
<!-- Card.svelte -->
<script>
    export let title;
    export let content;
</script>

<div class="card">
    <h2>{title}</h2>
    <p>{content}</p>
</div>

<!-- App.svelte -->
<script>
    import Card from './Card.svelte';
</script>

<Card title="Hello" content="World" />

```
## Conditional Rendering and Loops

```svelte

{#if user.loggedIn}
    <p>Welcome back, {user.name}!</p>
{:else}
    <button on:click={login}>Log in</button>
{/if}

<ul>
    {#each items as item, i}
        <li>{i + 1}: {item.name}</li>
    {/each}
</ul>
```

## Stores: Shared State


```svelte
<script>
  import { writable } from 'svelte/store';

  export const count = writable(0);

  // In any component:
  import { count } from './stores.js';

  $: console.log($count);

  function reset() {
    $count = 0;
  }
</script>
```

## Lifecycle Functions

| Function | Purpose |
|----------|---------|
| `onMount` | Run after component is first rendered |
| `onDestroy` | Cleanup when component unmounts |
| `beforeUpdate` | Run before DOM updates |
| `afterUpdate` | Run after DOM updates |

```svelte
<script>
    import { onMount } from 'svelte';

    onMount(() => {
        // Fetch data, set up intervals, etc.
        return () => {
            // Cleanup (runs on destroy)
        };
    });
</script>

```
## Bindings

Two-way data binding is straightforward:

```svelte

<input bind:value={name} />
<input type="checkbox" bind:checked={agreed} />
<select bind:value={selected}>
    {#each options as option}
        <option value={option}>{option}</option>
    {/each}
</select>
```

## Transition Animations


```svelte
<script>
  import { fade, fly } from 'svelte/transition';
  let visible = true;
</script>

{#if visible}
  <p transition:fade>Fades in and out</p>
  <p transition:fly={{ y: 200, duration: 500 }}>Flies in</p>
{/if}
```

Svelte's simplicity makes it an excellent choice for both small projects and large applications.
