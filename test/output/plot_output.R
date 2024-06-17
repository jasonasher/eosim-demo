library(tidyverse)

incidence_report <- read_csv("test/output/incidence_report.csv")
death_report <- read_csv("test/output/death_report.csv")

summarized_incidence_report <- incidence_report %>%
    group_by(scenario, day = floor(time)) %>%
    summarize(incidence = n()) %>%
    mutate(scenario = factor(scenario))

summarized_death_report <- death_report %>%
  group_by(scenario, day = floor(time)) %>%
  summarize(death = n()) %>%
  mutate(scenario = factor(scenario))

death_plot <- ggplot(summarized_death_report) +
  geom_line(aes(x = day, y = death, color = scenario)) +
  ggtitle("Death Report")

ggsave("test/output/death_report_plot.png", plot = death_plot)

incidence_plot <- ggplot(summarized_incidence_report) +
  geom_line(aes(x = day, y = incidence, color = scenario)) +
  ggtitle("Incidence Report")

ggsave("test/output/incidence_report_plot.png", plot = incidence_plot)