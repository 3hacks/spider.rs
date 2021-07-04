const INDEX = "change me";

async function run() {
  const {
    ClientBuilder
  } = require('@iota/client');

  const client = new ClientBuilder()
    .node('https://chrysalis-nodes.iota.org')
    .build();

  client.subscriber().topics(['messages']).subscribe((err, data) => {
    let payload;

    try {
      payload = JSON.parse(data.payload).payload;
    } catch(err) {
      return;
    }

    if (payload.type !== "Indexation") {
      return;
    }

    const index = payload.data.index.map((n) => {
      return String.fromCharCode(n);
    }).reduce((a, b) => {
      return a + b;
    });

    if (index === INDEX) {
      console.log(index);

      const data = payload.data.data.map((n) => {
        return String.fromCharCode(n);
      }).reduce((a, b) => {
        return a + b;
      });

      console.log(data);
    }
  })

  console.log("start");
}

run()
