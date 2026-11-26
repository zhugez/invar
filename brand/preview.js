const displacement = document.querySelector("#displacement");
const construction = document.querySelector(".construction-panel");
const displacementValue = document.querySelector("#displacement-value");

displacement.addEventListener("input", () => {
  construction.style.setProperty("--shift", `${displacement.value}px`);
  displacementValue.value = `${displacement.value} units`;
});
