console.time("test");
let random_numbers = [];

let hashMap = {};

for(let i = 0; i < 100000000; i++) {
  random_numbers[i] = Math.random();
} 

function seperate_into_bins() {
  let divide_number = 1/4096;

  for(let i = 0; i < 4096; i++) {
    hashMap[i] = [];
  }
  
  for(let i = 0; i < random_numbers.length; i++) {
    hashMap[Math.floor(random_numbers[i] / divide_number)].push(random_numbers[i]);
  }
}


seperate_into_bins();
const time = console.timeEnd("test");
// console.log(hashMap);

