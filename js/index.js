
/**
 * Super Simple Generic Function that applies all the rules to payload in order of priority
 * @param {Object} payload 
 * @param {Array<Object>} rules 
 * @returns {Object} payload 
 */
function apply_rules(args){
  const event = args['event'];
  const rules = args['rules'];
  rules.sort((a,b) => a.priority - b.priority)
        .map(rule => {
          var fn = new Function('return ' + rule['logic'])(); 
          fn(event) 
        });
  return args;
};