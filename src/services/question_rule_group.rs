use crate::{
    models::{
        answer::Answer,
        question_rule_group::{
            NewQuestionRuleGroup, NewQuestionRuleGroupWithRulesAndAnswers, QuestionRuleGroup,
            QuestionRuleGroupWithRulesAndAnswers,
        },
        question_rule_group_answer::QuestionRuleGroupAnswer,
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

pub fn create_question_rule_group(
    connection: &mut PgConnection,
    question_rule_group_info: NewQuestionRuleGroupWithRulesAndAnswers,
) -> Result<QuestionRuleGroupWithRulesAndAnswers, Error> {
    let new_rules = question_rule_group_info.rules;
    let answers_ids = question_rule_group_info.answers;

    let _question_rule_group: QuestionRuleGroup;
    match insert_into(questionrulegroups)
        .values::<NewQuestionRuleGroup>(NewQuestionRuleGroup {
            system_id: question_rule_group_info.system_id,
        })
        .get_result::<QuestionRuleGroup>(connection)
    {
        Ok(ok) => _question_rule_group = ok,
        Err(err) => return Err(err),
    };

    let _question_rule_group_rules: Vec<Rule>;
    match insert_into(rules::table)
        .values::<Vec<NewRule>>(
            new_rules
                .iter()
                .map(|rule_body| NewRule {
                    attribute_rule_group_id: None,
                    question_rule_group_id: Some(_question_rule_group.id),
                    compared_value: rule_body.compared_value.clone(),
                    logical_group: rule_body.logical_group,
                    operator: rule_body.operator.clone(),
                })
                .collect(),
        )
        .get_results::<Rule>(connection)
    {
        Ok(ok) => _question_rule_group_rules = ok,
        Err(err) => return Err(err),
    };

    let _answers: Vec<Answer>;
    match answers::table
        .filter(answers::id.eq_any(answers_ids))
        .load::<Answer>(connection)
    {
        Ok(ok) => _answers = ok,
        Err(err) => return Err(err),
    };

    let result = QuestionRuleGroupWithRulesAndAnswers {
        id: _question_rule_group.id,
        system_id: _question_rule_group.system_id,
        rules: _question_rule_group_rules,
        answers: _answers,
    };

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
