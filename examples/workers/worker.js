self.onmessage = ({ data }) => {
  const sum = data.reduce((total, value) => total + value, 0);
  self.postMessage(sum);
};
