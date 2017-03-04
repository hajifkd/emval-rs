var fntest = function() {
  return 1;
}

var obj = {
  one: 1,
  
  returnOne: function() {
    return this.one;
  },

  returnTrue: function() {
    return true;
  },

  returnFalse: function() {
    return false;
  },

  returnNot: function(b) {
    return !b;
  },

  helloWorld: function() {
    return "Hello, world!";
  },

  helloWorldMulti: function() {
    return "こんにちは、世界!";
  },

  helloSurrogatePair: function() {
    return "🍺";
  },

  add: function(x, y) {
    return x + y;
  }
};
