library(tidyverse)

plot_report <- function(df, variable) {
  summarized <- df %>%
    group_by(scenario, day = floor(time)) %>%
    summarize(value = n()) %>%
    mutate(scenario = factor(scenario))
  
  plot <- ggplot(summarized) +
    geom_line(aes(x = day, y = value, color = scenario)) +
    ggtitle(paste(variable, "report"))
  
  output_file <- paste("test/output/", variable, "_report_plot.png", sep = "")
  
  ggsave(output_file, plot = plot)
}

death_report <- read_csv("test/output/death_report.csv")
incidence_report <- read_csv("test/output/incidence_report.csv")

plot_report(death_report, "death")
plot_report(incidence_report, "incidence")
