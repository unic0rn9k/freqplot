using CSV
using DataFrames
using Statistics
using Printf

for path in filter(x -> occursin(r".*\.csv$", x), readdir("data/fløjte"))
    tone = CSV.read("./data/fløjte/" * path, DataFrame)

    println("----------------------------------------------")
    println(path)

    @printf "Mean frquency      : %.2f Hz\n" mean(tone.freq)
    @printf "Standart deviation : %.2f\n" std(tone.freq)

    println("----------------------------------------------")
    println()
end
