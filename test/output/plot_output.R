library(tidyverse)

incidence_report <- read_csv("test/output/incidence_report.csv")

summarized_incidence_report <- incidence_report %>%
    group_by(day = floor(time)) %>%
    summarize(incidence = n())

ggplot(summarized_incidence_report) +
    geom_line(aes(day, incidence))
