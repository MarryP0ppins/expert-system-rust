use crate::{
    models::{
        answer::Answer,
        question_rule_group::{
            NewQuestionRuleGroup, NewQuestionRuleGroupWithRulesAndAnswers, QuestionRuleGroup,
            QuestionRuleGroupWithRulesAndAnswers,
        },
        question_rule_group_answer::{NewQuestionRuleGroupAnswer, QuestionRuleGroupAnswer},
        rules::{NewRule, Rule},
    },
    schema::{answers, questionrulegroup_answer, questionrulegroups::dsl::*, rules},
};
use diesel::{delete, insert_into, prelude::*, result::Error};

pub fn get_question_rule_groups(
    connection: &mut PgConnection,
    system: i32,
) -> Result<Vec<QuestionRuleGroupWithRulesAndAnswers>, Error> {
    let _question_rule_group: Vec<QuestionRuleGroup>;
    match questionrulegroups
        .filter(system_id.eq(system))
        .load::<QuestionRuleGroup>(connection)
    {
        Ok(ok) => _question_rule_group = ok,
        Err(err) => return Err(err),
    };

    let _grouped_answers: Vec<Vec<(QuestionRuleGroupAnswer, Answer)>>;
    match QuestionRuleGroupAnswer::belonging_to(&_question_rule_group)
        .inner_join(answers::table)
        .select((questionrulegroup_answer::all_columns, answers::all_columns))
        .load::<(QuestionRuleGroupAnswer, Answer)>(connection)
    {
        Ok(ok) => _grouped_answers = ok.grouped_by(&_question_rule_group),
        Err(_) => _grouped_answers = vec![],
    };

    let _grouped_rules: Vec<Vec<Rule>>;
    match Rule::belonging_to(&_question_rule_group).load::<Rule>(connection) {
        Ok(ok) => _grouped_rules = ok.grouped_by(&_question_rule_group),
        Err(_) => _grouped_rules = vec![],
    };

    let result = _question_rule_group
        .into_iter()
        .zip(_grouped_answers)
        .zip(_grouped_rules)
        .map(
            |((_question_rule_group, _answers), _rules)| QuestionRuleGroupWithRulesAndAnswers {
                id: _question_rule_group.id,
                system_id: _question_rule_group.system_id,
                rules: _rules,
                answers: _answers.into_iter().map(|(_, answer)| answer).collect(),
            },
        )
        .collect::<Vec<QuestionRuleGroupWithRulesAndAnswers>>();

    Ok(result)
}

pub fn create_question_rule_groups(
    connection: &mut PgConnection,
    question_rule_group_info: Vec<NewQuestionRuleGroupWithRulesAndAnswers>,
) -> Result<Vec<QuestionRuleGroupWithRulesAndAnswers>, Error> {
    let (new_rules, answers_ids, question_rule_group_system_ids) = question_rule_group_info
        .into_iter()
        .fold((vec![], vec![], vec![]), |mut acc, raw| {
            acc.0.push(raw.rules);
            acc.1.push(raw.answers);
            acc.2.push(NewQuestionRuleGroup {
                system_id: raw.system_id,
            });
            acc
        });

    let _question_rule_group: Vec<QuestionRuleGroup>;
    match insert_into(questionrulegroups)
        .values::<Vec<NewQuestionRuleGroup>>(question_rule_group_system_ids)
        .get_results::<QuestionRuleGroup>(connection)
    {
        Ok(ok) => _question_rule_group = ok,
        Err(err) => return Err(err),
    };

    let _grouped_rules: Vec<Vec<Rule>>;
    match insert_into(rules::table)
        .values::<Vec<NewRule>>(
            new_rules
                .into_iter()
                .zip(&_question_rule_group)
                .flat_map(|(rules, group)| {
                    rules.into_iter().map(|rule| NewRule {
                        attribute_rule_group_id: None,
                        question_rule_group_id: Some(group.id),
                        compared_value: rule.compared_value.clone(),
                        logical_group: rule.logical_group,
                        operator: rule.operator.clone(),
                    })
                })
                .collect(),
        )
        .get_results::<Rule>(connection)
    {
        Ok(ok) => _grouped_rules = ok.grouped_by(&_question_rule_group),
        Err(err) => return Err(err),
    };

    match insert_into(questionrulegroup_answer::table)
        .values::<Vec<NewQuestionRuleGroupAnswer>>(
            answers_ids
                .into_iter()
                .zip(&_question_rule_group)
                .flat_map(|(answers_ids, group)| {
                    answers_ids
                        .into_iter()
                        .map(|value| NewQuestionRuleGroupAnswer {
                            answer_id: value,
                            question_rule_group_id: group.id,
                        })
                })
                .collect(),
        )
        .execute(connection)
    {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    let _grouped_answers: Vec<Vec<(QuestionRuleGroupAnswer, Answer)>>;
    match QuestionRuleGroupAnswer::belonging_to(&_question_rule_group)
        .inner_join(answers::table)
        .select((questionrulegroup_answer::all_columns, answers::all_columns))
        .load::<(QuestionRuleGroupAnswer, Answer)>(connection)
    {
        Ok(ok) => _grouped_answers = ok.grouped_by(&_question_rule_group),
        Err(_) => _grouped_answers = vec![],
    };

    let result = _question_rule_group
        .into_iter()
        .zip(_grouped_answers)
        .zip(_grouped_rules)
        .map(|((_question_rule_group, _answers_values), _rules)| {
            QuestionRuleGroupWithRulesAndAnswers {
                id: _question_rule_group.id,
                system_id: _question_rule_group.system_id,
                rules: _rules,
                answers: _answers_values
                    .into_iter()
                    .map(|(_, answers)| answers)
                    .collect(),
            }
        })
        .collect::<Vec<QuestionRuleGroupWithRulesAndAnswers>>();

    Ok(result)
}

pub fn multiple_delete_question_rule_groups(
    connection: &mut PgConnection,
    question_rule_groups_ids: Vec<i32>,
) -> Result<usize, Error> {
    match delete(questionrulegroups.filter(id.eq_any(question_rule_groups_ids))).execute(connection)
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
