const registerSwitch = document.getElementById('switch') as HTMLInputElement;

function swapForm() {
  if (registerSwitch.checked) {
    document.getElementById('registerForm')!.style.display = 'block';
    document.getElementById('loginForm')!.style.display = 'none';
  } else {
    document.getElementById('registerForm')!.style.display = 'none';
    document.getElementById('loginForm')!.style.display = 'block';
  }
}

swapForm();

registerSwitch.addEventListener('change', () => {
  swapForm();
});
