## Connection

Available teams
```yaml
# serv -> client
type: available_teams
payload:
  teams: List[str]
```

Join request: if team exists, join team, otherwise create a new one
```yaml
# client -> serv
type: join_request
payload:
  username: str
  team: str
```

Join confirmed
```yaml
# serv -> client
type: joined_game
```

Player connected
```yaml
# serv -> client
type: player_connected
payload:
  username: str
  team: str
```

Game start request
```yaml
# client -> serv
type: game_start_request
payload:
  username: str  # the user who requested the start
```

Game started
```yaml
# serv -> client
type: game_started
payload:
  username: str  # the user who requested the start
```


## Gameplay

Poney dragged
```yaml
# client -> serv
type: poney_dragged
payload:
  team: str
```

Items available
```yaml
# serv -> client
type: items_available
payload:
  items: List[Item]

Item:
  name: str  # unique, used as key
  price: float
```

Budget update: only member team's budget is updated
```yaml
# client -> serv
type: budget_udpate
payload:
  budget: float
```

Item buy request
```yaml
# client -> serv
type: item_buy_request
payload:
  item: str
```

Item buy confirm
```yaml
# client -> serv
type: item_buy_confirm
payload:
  item: str
```

Poney position
```yaml
# serv client
type: poney_position
payload:
  position: float
```

Game over
```yaml
# serv -> client
type: game_over
payload:
  winner: int
```
