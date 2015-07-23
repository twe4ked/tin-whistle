class TinWhistle
  NOTES = {
    'd' => [1, 1, 1, 1, 1, 1],
    'e' => [1, 1, 1, 1, 1, 0],
    'f' => [1, 1, 1, 1, 0, 0], # sharp
    'g' => [1, 1, 1, 0, 0, 0],
    'a' => [1, 1, 0, 0, 0, 0],
    'b' => [1, 0, 0, 0, 0, 0],
    'c' => [0, 0, 0, 0, 0, 0], # sharp
    'D' => [0, 1, 1, 1, 1, 1],
  }

  def initialize
    notes = STDIN.read.lines.reject do |line|
      line =~ /^T:/ ||
      line =~ /^#/
    end.join

    @input = notes
  end

  def print
    lines = @input.lines
    number_of_lines = lines.size
    lines.each_with_index do |line, index|
      line = line.chars.select { |c| c != '\n' }
      print_line(line) unless line.empty?
      puts unless index+1 == number_of_lines
    end
  end

  def print_line(line)
    # print out the name of the note
    line.each do |note|
      str = if ['f', 'F', 'c', 'C'].includes?(note)
        "#{note}#"
      else
        note.to_s
      end

      printf("%-3s", str)
    end
    puts

    # print a "." if the note is in the upper octave
    line.each do |note|
      char = if ('a'..'g').to_a.includes?(note)
        '.'
      elsif note == '|'
        '|'
      end
      printf("%-3s", char)
    end
    puts

    6.times do |n|
      line.each do |note|
        char = case
        when ('a'..'g').to_a.includes?(note.downcase)
          if NOTES[note.downcase][n] == 1
            '●'
          else
            '○'
          end
        when [' ',  '.', '|'].includes?(note)
          note
        end

        printf("%-3s", char)
      end
      puts
    end
  end
end

TinWhistle.new.print
