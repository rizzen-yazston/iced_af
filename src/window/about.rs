// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, WindowType,
        constants::{APPLICATION_NAME, AUTHORS, VERSION},
        StringGroup,
    },
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, WindowTrait},
    },
    localisation::about::{Index, Strings},
};
use iced::{
    widget::{button, column, row, scrollable, text},
    window, Alignment, Task, Element, Length,
};
use std::any::Any;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct State {
    contributors: Vec<String>,
    localisation_contributors: Vec<String>,
}

impl State {
    pub fn try_new(localisation: &Localisation) -> Result<Self, ApplicationError> {
        let mut split = AUTHORS.split(',');
        let mut contributors = Vec::<String>::new();
        while let Some(author) = split.next() {
            contributors.push(author.trim().to_string());
        }
        let details = localisation.repository_details()?;
        let localisation_contributors = details.contributors.clone();
        Ok(State {
            contributors,
            localisation_contributors,
        })
    }
}

impl AnyWindowTrait for State {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl WindowTrait for State {
    fn window_type(&self) -> WindowType {
        WindowType::About
    }

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String {
        let strings = string_cache.get(&StringGroup::About).unwrap();
        strings.title()
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let align_start = localisation.layout_data().align_words_start;
        let reverse_words = localisation.layout_data().reverse_words;
        let reverse_lines = localisation.layout_data().reverse_lines;
        let strings = string_cache.get(&StringGroup::About).unwrap();
        let mut content: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();

        // Header
        #[allow(unused_mut)]
        let mut header: Vec<Element<application::Message>> =
            vec![APPLICATION_NAME.into(), VERSION.into()];
        if reverse_lines {
            header.reverse();
        }
        content.push(
            column(header)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
        );

        // Body - scrollable
        let mut body: Vec<Element<application::Message>> = Vec::<Element<application::Message>>::new();
        let mut contributors: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();
        let iterator = self.contributors.iter();
        for author in iterator {
            #[allow(unused_mut)]
            let mut contributor: Vec<Element<application::Message>> =
                vec![text("  ").into(), text(author.clone()).into()];
            if reverse_words {
                contributor.reverse();
            }
            contributors.push(row(contributor).into());
        }
        if reverse_lines {
            contributors.reverse();
        }
        body.push(text(strings.string(Index::Contributors as usize)).into());
        body.push(
            column(contributors)
                .width(Length::Fill)
                .align_x(align_start)
                .into(),
        );
        let mut localisations: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();
        let iterator = self.localisation_contributors.iter();
        for language in iterator {
            let mut contributor: Vec<Element<application::Message>> = vec![
                text("  ").into(), // Indentation space
                text(language.clone()).into(),
            ];
            if reverse_words {
                contributor.reverse();
            }
            localisations.push(row(contributor).into());
        }
        if reverse_lines {
            localisations.reverse();
        }
        body.push(" ".into()); // Paragraph separation
        body.push(text(strings.string(Index::Localisation as usize)).into());
        body.push(
            column(localisations)
                .width(Length::Fill)
                .align_x(align_start)
                .into(),
        );
        if reverse_lines {
            body.reverse();
        }
        content.push(
            scrollable(column(body).width(Length::Fill).align_x(align_start))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        );
        content.push(" ".into()); // Paragraph separation

        // OK button
        content.push(
            column![button(text(strings.string(Index::Ok as usize)))
                .padding([5, 10])
                .on_press(application::Message::Close(id))]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into(),
        );
        if reverse_lines {
            content.reverse();
        }
        column(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .into()
    }

    fn reusable(&self) -> bool {
        true
    }

    fn global_disable(&self) -> bool {
        true
    }
}

pub fn display(
    application: &mut application::State,
    parent: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    if !application.string_cache.exists(&StringGroup::About) {
        application.string_cache.insert(
            StringGroup::About,
            Box::new(Strings::try_new(&application.localisation)?),
        );
    }
    let state: Box<dyn AnyWindowTrait> = match application.manager.use_reusable(WindowType::About) {
        None => Box::new(State::try_new(&application.localisation)?),
        Some(value) => value,
    };
    Ok(application
        .manager
        .try_spawn(&mut application.session, state, parent)?)
}
