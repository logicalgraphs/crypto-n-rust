// add server.js to the parent directory of protocols to run

// to add express package, 
// run 'npm init -y' 
// then, run `npm install express` 
// in the terminal, then run `node server.js` to start the server, 
// and open http://localhost:3000 in the browser to view the app

const express = require('express');
const path = require('path');
const app = express();
const port = 3000;
const args = process.argv.slice(2);

Object.defineProperty(Array.prototype, 'first', {
  value() {
    return this.find(Boolean)
  }
});

const dispatch = args.first() || 'pivoteur';
const appName = dispatch + '.github.io';
const vennDir = appName + '/.git/modules/charts';

app.use(express.static(path.join(__dirname, appName)));
// app.use('/data', express.static(path.join(__dirname, 'data')));
// app.use('/imgs', express.static(path.join(__dirname, 'imgs')));

app.get('/', (req, res) => {
  res.sendFile(path.join(__dirname, appName, 'index.html'));
});

app.listen(port, () => {
  console.log('App is', appName);
  console.log(`Server is running at http://localhost:${port}`);
});

