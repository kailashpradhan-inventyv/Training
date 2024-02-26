const fs = require('fs');
const file=require("./pkg/rust_web_assembly.js");

fs.readFile('JSON-data/data.json', 'utf8', (err, data) => {
  if (err) {
    console.error('Error reading the file:', err);
    return;
  }

  try {
    const out_data=file.table_value_update(data);
    console.log(out_data);
  } catch (parseError) {
    console.error('Error parsing JSON:', parseError);
  }
});


