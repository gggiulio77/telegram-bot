# telegram-bot

Hello there! This project has been a valuable learning opportunity for [Teloxide](https://github.com/teloxide/teloxide), [SurrealDb](https://surrealdb.com/), [Knative](https://knative.dev/docs/), and [Rust](https://www.rust-lang.org/). It encompasses an event-driven strategy for hosting a [Telegram bot](https://core.telegram.org/bots/api) deployed within [Kubernetes](https://kubernetes.io/) and supported by [Knative](https://knative.dev/docs/).

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

## Getting Started


The bot's aim is to save a user's chat ID and email in a database, which is achieved through the `/start` command. After this process, we utilize [telegram-sender](https://github.com/gggiulio77/telegram-sender) for an event-driven approach to sending messages to users.

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

For running it locally, you'll need to deploy a [SurrealDb database](https://surrealdb.com/docs/surrealdb/installation/running/docker) and obtain a [Telegram bot token](https://core.telegram.org/bots/tutorial#obtain-your-bot-token).

Alternatively, you will need a [Kubernetes](https://kubernetes.io/) cluster with [Knative installed](https://knative.dev/docs/install/) to run it in a cloud environment.

### Installation

`git clone https://github.com/gggiulio77/telegram-bot.git`

## Usage

For local execution, simply create a `.env` file containing all required information.

This bot is developed using the [Telegram webhook method](https://core.telegram.org/bots/webhooks). Therefore, it's necessary to store a publicly accessible URL with certificates in the `TELOXIDE_URL` environment variable. Alternatively, you can refactor the code to utilize the [polling method](https://core.telegram.org/bots/api#getting-updates) to avoid this requirement.

For deployment in Knative, utilize `kubectl apply -f knative.server.service.yml`. Certain environment values are stored in [Kubernetes Secrets](https://kubernetes.io/docs/concepts/configuration/secret/), thus you'll need to refactor the file or create secrets accordingly.

If you have a local image repository on port `32000`, you can utilize the `build.sh` script in the scripts folder to build the project and update `knative.server.service.yml` with the new image digest.

## Roadmap

- [ ] Refactor start command to use user's email 
- [ ] Improve documentation 
- [ ] Include a docker-compose file for local execution
- [ ] Update axum version (breaking changes)
- [ ] Think about replacing Teloxide with other library

## License
