library(tidyverse)

incidence_report <- read_csv("test/output/incidence_report.csv")

summarized_incidence_report <- incidence_report %>%
    group_by(scenario, day = floor(time)) %>%
    summarize(incidence = n()) %>%
    mutate(scenario = factor(scenario))

ggplot(summarized_incidence_report) +
    geom_line(aes(x=day, y=incidence, color=scenario))
