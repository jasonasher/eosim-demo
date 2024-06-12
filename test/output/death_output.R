library(tidyverse)

death_report <- read_csv("test/output/death_report.csv")

summarized_death_report <- death_report %>%
  group_by(scenario, day = floor(time)) %>%
  summarize(death = n())

ggplot(summarized_death_report) +
  geom_line(aes(day, death, group = scenario))