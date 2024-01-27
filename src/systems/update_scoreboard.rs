use bevy::{
    app::{Plugin, Update}, ecs::{
        query::With, schedule::{common_conditions::in_state, IntoSystemConfigs, ScheduleLabel}, system::{Query, Res}
    }, text::Text, utils::intern::Interned
};

use crate::{
    components, resources, states
};

pub struct UpdateScoreboardPlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for UpdateScoreboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        match self.state {
            Some(s) => app.add_systems(self.schedule, update_scoreboard.run_if(in_state(s))),
            None => app.add_systems(self.schedule, update_scoreboard),
        };
    }
}

fn update_scoreboard(scoreboard: Res<resources::Scoreboard>, mut query: Query<&mut Text, With<components::Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
