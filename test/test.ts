const res = await fetch("http://localhost:3030/greeting", {
  headers: {
    Authorization: `Emoji ${encodeURIComponent("🐱🐶🐧🐤")}`,
  },
});
const text = await res.text();
console.log(text);
