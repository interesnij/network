
on('#ajax', 'click', '.load_attach_survey_list', function() {
  profile_list_block_attach(this, "/surveylist/", "load_attach_survey_list");
});

on('#ajax', 'click', '.load_survey_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  surveylist_pk = parent.getAttribute("surveylist-pk");
  create_fullscreen("/surveys/load_list/" + surveylist_pk, "item_fullscreen", false, true);
});

on('#ajax', 'click', '.add_survey', function() {
  create_fullscreen('/survey/add_survey_in_list/' + this.parentElement.getAttribute("data-pk"), "worker_fullscreen", true, true);
});
on('#ajax', 'click', '.survey_edit', function() {
  create_fullscreen('/survey/edit/' + this.parentElement.parentElement.parentElement.getAttribute("data-pk"), "worker_fullscreen", true, true);
});

on('#ajax', 'click', '.survey_info', function() {
  create_fullscreen('/survey/voters/' + this.parentElement.parentElement.parentElement.getAttribute("data-pk"), "worker_fullscreen");
});

on('#ajax', 'click', '#need_time_end', function() {
  this.parentElement.parentElement.nextElementSibling.classList.toggle("hide")
});

on('#ajax', 'click', '.remove_answer', function() {
  this.parentElement.parentElement.parentElement.remove()
});
on('#ajax', 'click', '.add_answer', function() {
  container = this.parentElement.parentElement.parentElement.parentElement
  answers = container.querySelectorAll(".answer");
  answers.length > 9 ? toast_error("Допустимо не больше 10 вариантов!") :
  (div = document.createElement("div"), div.classList.add("form-group"),
  div.innerHTML = '<div class="input-group"><div class="input-group-prepend"><span class="input-group-text handle">≡</span></div><input type="text" name="answers" placeholder="Вариант ответа" class="form-control answer"><div class="input-group-append"><span class="input-group-text custom_color pointer remove_answer">x</span></div></div>',
  container.append(div));
});
