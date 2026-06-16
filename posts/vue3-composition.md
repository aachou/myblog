+++
title = "Vue 3 Composition API: When and How to Use It"
date = "2026-03-05"
tags = ["vue", "javascript", "frontend"]
excerpt = "The Composition API is powerful but not always necessary. This guide covers practical patterns for deciding when to reach for it and how to organize your code effectively."
+++

When Vue 3 shipped the Composition API, it introduced a paradigm shift that divided the community. Some adopted it eagerly, while others felt the Options API was perfectly fine. The truth is both have their place.

## The `setup` Function

The Composition API centers on the `setup` function, which runs before the component is created.

```vue
<script setup>
import { ref, computed, onMounted } from "vue";

const count = ref(0);
const double = computed(() => count.value * 2);

function increment() {
  count.value++;
}

onMounted(() => {
  console.log(`Initial count: ${count.value}`);
});
</script>
```

The `<script setup>` syntax is the recommended way to use the Composition API. It reduces boilerplate and improves TypeScript inference.

## When to Use Composition over Options

The Options API shines for simple components where data, methods, and computed properties are few. Reach for the Composition API when:

1. **Logic reuse** 鈥?you want to share reactive state between components
2. **Complex components** 鈥?a single component has too many concerns mixed together
3. **TypeScript** 鈥?you want better type inference without extra annotations

```javascript
// useCounter.js 鈥?a composable
import { ref, computed } from "vue";

export function useCounter(initial = 0) {
  const count = ref(initial);
  const double = computed(() => count.value * 2);

  function increment() {
    count.value++;
  }

  return { count, double, increment };
}
```

This composable can now be consumed by any component with a single line:

```javascript
const { count, double, increment } = useCounter(10);
```

## Organizing Composition Functions

A common mistake is dumping everything into `setup` without structure. Group related logic together:

```vue
<script setup>
import { useUser } from "@/composables/useUser";
import { useNotifications } from "@/composables/useNotifications";
import { useForm } from "@/composables/useForm";

const { user, isLoading } = useUser();
const { notify } = useNotifications();
const { form, submit, errors } = useForm();
</script>
```

Each composable should have a single responsibility. If a function does more than one thing, split it.

## Reactive References and Destructuring

One of the trickiest parts of the Composition API is understanding when reactivity is preserved.

```javascript
// 鉂?Loses reactivity
const { count } = useCounter();

// 鉁?Keeps reactivity
const { count } = useCounter(); // count is a ref, use .value
```

If a composable returns a `ref`, destructuring works because you are still holding a reference to the `Ref` object. But if it returns a plain value derived from reactive data, that value will be stale.

## Watchers with Fine-Grained Control

```javascript
const searchQuery = ref("");

watch(searchQuery, async (newQuery, oldQuery) => {
  if (newQuery.length < 3) return;
  results.value = await fetchResults(newQuery);
}, { debounce: 300 });
```

The `watch` function offers explicit control over what triggers side effects, unlike `watchEffect` which automatically tracks all reactive dependencies.

## Lifecycle Hooks

Lifecycle hooks in the Composition API mirror the Options API but are prefixed with `on`:

| Options API | Composition API |
|---|---|
| `mounted` | `onMounted` |
| `updated` | `onUpdated` |
| `beforeUnmount` | `onBeforeUnmount` |
| `errorCaptured` | `onErrorCaptured` |

There is no `beforeCreate` or `created` because those are replaced by `setup` itself.

## Migration Strategy

If you are migrating an existing Vue 2 project, do not rewrite everything at once. You can use the Composition API alongside the Options API in the same component. Start with the most complex components and work your way down.

## Conclusion

The Composition API is not a replacement for the Options API 鈥?it is a complementary tool. Use it for logic reuse, complex state management, and TypeScript-heavy projects. For simple presentational components, the Options API remains clean and readable.
