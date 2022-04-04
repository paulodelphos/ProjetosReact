defmodule Ex003 do

  def run() do

    n1 = IO.gets("Digite o primeiro valor: ")
    {num1, _} = Integer.parse(n1)

    #|>IO.inspect()

    #n2 = IO.gets("Digite o segundo valor: ")
    #{num2, _} = Integer.parse(n2)
    #total = num1 + num2
    #|>n1 = parse_input()
    IO.gets("Digite o segundo valor:")
    |>Integer.parse()
    |>parse_input()
    |>somaDois(num, num1)
    |>print()

  end

  def print(total) do
    IO.puts("valor foi #{total}")
  end

  def somaDois(num, num1) do
    total = num + num1
    total
  end

  def parse_input(data) do
    if data == :error do
      IO.puts("Invalid level!!")
      run()
    else
      {num, _} = data
      num
    end
  end

end
