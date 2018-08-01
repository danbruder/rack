function submitForm(form) {
  $.ajax({
    url: form.action,
    type: form.method,
    dataType: 'script',
    data: $(form).serialize(),
  });
}

document.addEventListener('turbolinks:load', function() {
  $('form').submit(function(e) {
    e.preventDefault();
    submitForm(e.target);
  });
});
