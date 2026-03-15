use crate::{
    db::repositories::{ContributorWithStats, RepositoryRecord},
    galaxy::{
        layout::{AsteroidBelt, GalaxyLayout, Planet, Star},
        orbit,
    },
};

pub struct GalaxyGenerator;

impl GalaxyGenerator {
    pub fn generate(repo: &RepositoryRecord, contributors: &[ContributorWithStats]) -> GalaxyLayout {
        const MAX_PLANETS: usize = 50;

        let star_size = 4.5 + (repo.stars.max(1) as f32).log10();
        let brightness = (0.5 + (repo.forks.max(1) as f32).log10() / 5.0).clamp(0.5, 1.0);

        // 1) Sort contributors by commits (descending), then by username for stability.
        let mut sorted = contributors.to_vec();
        sorted.sort_by(|a, b| {
            b.commits
                .cmp(&a.commits)
                .then_with(|| a.username.cmp(&b.username))
        });

        let total_contributors = sorted.len();
        let asteroid_count = total_contributors.saturating_sub(MAX_PLANETS);

        // 2) Keep only the top N contributors as planets.
        let planets = sorted
            .iter()
            .take(MAX_PLANETS)
            .enumerate()
            .map(|(idx, contributor)| {
                let radius = orbit::orbit_radius(idx + 1);
                let seed = format!("{}:{}:{}", repo.full_name, contributor.username, contributor.commits);
                let position = orbit::deterministic_position(&seed, radius);
                let size = planet_size_from_commits(contributor.commits);

                Planet {
                    username: contributor.username.clone(),
                    size,
                    position,
                    commits: contributor.commits,
                }
            })
            .collect();

        GalaxyLayout {
            star: Star {
                name: repo.full_name.clone(),
                size: star_size,
                brightness,
            },
            planets,
            asteroid_belt: AsteroidBelt {
                count: asteroid_count,
            },
        }
    }
}

fn planet_size_from_commits(commits: i32) -> f32 {
    let commits = (commits.max(0) + 1) as f32;
    let scale_factor = 1.3_f32;
    commits.ln() * scale_factor
}
