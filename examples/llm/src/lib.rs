// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/*!
# LLM Example Application

This example application runs a large language model in an application's service.

NOTE: Due to [lack of hardware acceleration](https://github.com/linera-io/linera-protocol/issues/1931) performance is wanting.

# How It Works

Models and tokenizers are served locally using a local Python server. They are expected
at `model.bin` and `tokenizer.json`.

The application's service exposes a single GraphQL field called `prompt` which takes a prompt
as input and returns a response.

When a prompt is submitted, the application's service uses the `fetch_url`
system API to inject the model and tokenizer. Subsequently, the model bytes are converted
to the GGUF format where it can be used for inference.

# Usage

## Setting Up

First ensure you have the model and tokenizer locally by running:

```bash
wget -O model.bin -c https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/tree/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf
wget -c https://huggingface.co/spaces/lmz/candle-llama2/resolve/main/tokenizer.json
```

Then in a separate terminal window, run the Python server to serve models locally:
```bash
python3 -m http.server 10001
```

Finally, deploy the application:

```bash
cd ../ && linera project publish-and-create llm
```

## Using the LLM Application

First, a node service for the current wallet has to be started:

```bash
PORT=8080
linera service --port $PORT &
```

Next, navigate to `llm/web-frontend` and install the requisite `npm`
dependencies:

```bash
cd llm/web-frontend
npm install
npm start
```

Finally, navigate to `localhost:3000` to interact with the Linera ChatBot.
 */

use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct LlmAbi;

impl ContractAbi for LlmAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for LlmAbi {
    type Query = Request;
    type QueryResponse = Response;
}
