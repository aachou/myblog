+++
title = "GraphQL Subscriptions"
date = "2022-04-05"
tags = ["graphql", "real-time", "nodejs"]
excerpt = "Learn how to implement real-time updates in GraphQL using subscriptions over WebSockets. Covers setup, resolvers, and authentication."
+++

GraphQL subscriptions provide a way to push real-time updates from the server to the client. While queries fetch data and mutations modify it, subscriptions maintain an active connection for live updates.

## WebSocket Transport

Subscriptions typically use WebSockets instead of HTTP. The Apollo Server ecosystem provides `graphql-ws` for this purpose.

```javascript

import { WebSocketServer } from "ws";
import { useServer } from "graphql-ws/lib/use/ws";

const wsServer = new WebSocketServer({
    server: httpServer,
    path: "/graphql",
});

useServer({ schema }, wsServer);
```

## Defining a Subscription

Subscriptions are defined in the schema just like queries and mutations:

```graphql
type Subscription {
    messageAdded(roomId: ID!): Message
}

type Message {
    id: ID!
    content: String
    author: String
}
```

## Pub/Sub Systems

The server needs a publish-subscribe mechanism to broadcast events. In-memory `EventEmitter` works for single-process servers, but production systems use Redis.


```javascript
import { PubSub } from "graphql-subscriptions";

const pubsub = new PubSub();

const resolvers = {
  Subscription: {
    messageAdded: {
      subscribe: (_, { roomId }) =>
        pubsub.asyncIterator([`MESSAGE_ADDED_${roomId}`]),
    },
  },
  Mutation: {
    addMessage: (_, { roomId, content, author }) => {
      const message = { id: uuid(), content, author };
      pubsub.publish(`MESSAGE_ADDED_${roomId}`, { messageAdded: message });
      return message;
    },
  },
};
```

## Client-Side Usage

On the client, use `useSubscription` from `@apollo/client`:

```javascript
function MessageList({ roomId }) {
    const { data, loading } = useSubscription(
        gql`
            subscription OnMessageAdded($roomId: ID!) {
                messageAdded(roomId: $roomId) {
                    id
                    content
                    author
                }
            }
        `,
        { variables: { roomId } }
    );

    if (loading) return <p>Waiting for messages...</p>;
    return <div>{data?.messageAdded.content}</div>;
}

```
## Authentication

Authenticate the WebSocket connection during the upgrade handshake:

```javascript

const server = useServer({
    schema,
    onConnect: (ctx) => {
        const token = ctx.connectionParams?.authToken;
        if (!isValid(token)) throw new Error("Unauthorized");
        return { currentUser: decode(token) };
    },
}, wsServer);
```

## Scaling

For multi-process deployments, replace the in-memory pubsub with Redis:


```javascript
import { RedisPubSub } from "graphql-redis-subscriptions";
const pubsub = new RedisPubSub();
```

Subscriptions transform GraphQL from a request-response protocol into a full duplex communication channel.
