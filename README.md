# Concierge

Description

    Version : 0.1.0

# Liste des commandes : 

✔️ = Disponible | ❌ = Non compatible | 🔁 = Prévu

| Commande | Description | Linux | Windows |
| :---------------: | :---------------: | :---------------: | :-----: |
| uptime | Uptime de l'hôte | ✔️ | ✔️ |
| load | Load average de l'hôte | ✔️ | ❌ |
| memory | Utilsation de la mémoire de l'hôte | ✔️ | ✔️ |
| mounts | Liste des points de montage | ✔️ | ✔️ |
| networks | Liste des interfaces réseau | 🔁 | 🔁 |
| socket_stats | Statistique des sockets réseau | 🔁 | 🔁 |
| network_stats | Statistique du réseau | 🔁 | 🔁 |
| disk_stats | Statistique des disques | 🔁 | 🔁 |
| cpu_stats | Statistique du CPU | 🔁 | 🔁 |
| exit | Ferme la connexion TCP | ✔️ | ✔️ |



# Format de réponse
```json
{
    "status": "ok",
    "data": "data"
}
```

# Liste des données de réponse
## uptime
