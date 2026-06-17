+++
title = "Elixir Phoenix"
date = "2022-09-14"
tags = ["elixir", "phoenix", "web-framework"]
excerpt = "Building modern web applications with Elixir and Phoenix. Explore channels, Ecto, and the legendary OTP-based concurrency model."
+++

Phoenix is a web framework for Elixir that brings together the power of the Erlang VM with a productive, Ruby-like syntax. It excels at real-time applications and fault-tolerant systems.

## Why Phoenix?

- **Performance** 鈥?handles millions of concurrent connections
- **Fault tolerance** 鈥?built on OTP supervisors
- **Real-time** 鈥?first-class WebSocket support via Channels
- **Productivity** 鈥?mix tasks, live reload, and comprehensive testing

## Project Structure

```bash
mix phx.new my_app
cd my_app
mix ecto.create
mix phx.server
```

A new Phoenix project gives you:

```
my_app/
鈹溾攢鈹€ lib/
鈹?  鈹溾攢鈹€ my_app/
鈹?  鈹?  鈹溾攢鈹€ accounts/     # Context
鈹?  鈹?  鈹溾攢鈹€ blog/         # Context
鈹?  鈹?  鈹斺攢鈹€ ...
鈹?  鈹斺攢鈹€ my_app_web/
鈹?      鈹溾攢鈹€ controllers/
鈹?      鈹溾攢鈹€ views/
鈹?      鈹溾攢鈹€ templates/
鈹?      鈹斺攢鈹€ channels/
鈹溾攢鈹€ test/
鈹斺攢鈹€ config/
```

## Ecto Queries

Ecto is Phoenix's database wrapper and query generator:

```elixir
defmodule MyApp.Blog do
    import Ecto.Query

    def list_published_posts do
        Repo.all(
            from p in Post,
                where: p.published == true,
                order_by: [desc: p.published_at],
                preload: [:author, :tags]
        )
    end
end

```
## Phoenix Channels

Channels provide real-time bidirectional communication:

```elixir

defmodule MyAppWeb.RoomChannel do
    use Phoenix.Channel

    def join("room:" <> room_id, _payload, socket) do
        {:ok, assign(socket, :room_id, room_id)}
    end

    def handle_in("new_message", %{"body" => body}, socket) do
        broadcast!(socket, "new_message", %{user: socket.assigns.user, body: body})
        {:noreply, socket}
    end
end
```

## LiveView

LiveView enables real-time UI without writing JavaScript:


```elixir
defmodule MyAppWeb.CounterLive do
  use Phoenix.LiveView

  def render(assigns) do
    ~H"""
    <div>
      <h1>Count: <%= @count %></h1>
      <button phx-click="inc">+</button>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    {:ok, assign(socket, :count, 0)}
  end

  def handle_event("inc", _, socket) do
    {:noreply, update(socket, :count, &(&1 + 1))}
  end
end
```

## OTP and Supervisors

Everything in Phoenix runs under a supervision tree. If a process crashes, the supervisor restarts it:

```elixir
children = [
    MyApp.Repo,
    MyAppWeb.Endpoint,
    {MyApp.SomeWorker, [name: :worker]}
]

opts = [strategy: :one_for_one, name: MyApp.Supervisor]
Supervisor.start_link(children, opts)

```
## Testing

```elixir
test "GET /", %{conn: conn} do
  conn = get(conn, "/")
  assert html_response(conn, 200) =~ "Welcome"
end
```

Phoenix and Elixir offer a refreshing alternative to the JavaScript-dominated web ecosystem.
