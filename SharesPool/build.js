const fs = require('fs');
const solc = require('solc');

const contractPath = 'SharesPool.sol';
const contractContent = fs.readFileSync(contractPath, 'utf8');
const solcConfig = require('./solc-config.json');

const compiledContract = solc.compile(JSON.stringify(solcConfig));

fs.writeFileSync('SharesPool.json', compiledContract);