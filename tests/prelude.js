var fntest = function() {
  return 1;
}

var obj = {
  one: 1,
  
  returnOne: function() {
    return this.one;
  },

  helloWorld: function() {
    return "Hello, world!";
  },

  helloWorldMulti: function() {
    return "こんにちは、世界!";
  },

  add: function(x, y) {
    console.log(x);
    console.log(y);
    return x + y;
  }
};
