# Concierge

Description

    Version : 0.1.0

# Liste des commandes : 

| Commande | Description | Linux | Windows |
| :---------------: | :---------------: | :---------------: | :-----: |
| uptime | Uptime de l'hôte | ✔️ | ✔️ |
| load | Load average de l'hôte | ✔️ | ❌ |
| memory | Utilsation de la mémoire de l'hôte | ✔️ | ✔️ |
| mounts | Liste des points de montage | ✔️ | ✔️ |
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
