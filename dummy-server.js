const express = require('express');
const app = express();

// Define a route
app.get('/', (req, res) => {
  res.send('Hello from the dummy server!');
});

// Start the server
app.listen(9000, () => {
  console.log('Dummy server is running on port 9000');
});
