defmodule Ex003 do

  def main() do

    IO.gets("Digite o primeiro valor: ")
    |>Integer.parse()
    |>parse_input(data)
    |>IO.inspect()

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
