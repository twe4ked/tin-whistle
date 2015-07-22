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

  def initialize(input)
    @input = input.select { |c| c != '\n' }
  end

  def print
    # print out the name of the note
    @input.each do |note|
      char = if ['f', 'F', 'c', 'C'].includes?(note)
        "#{note}#"
      else
        note.to_s
      end

      printf("%-3s", char)
    end
    puts

    # print a "." if the note is in the upper octave
    @input.each do |note|
      char = if ('A'..'Z').to_a.includes?(note)
        '.'
      elsif note == '|'
        '|'
      end
      printf("%-3s", char)
    end
    puts

    6.times do |n|
      @input.each do |note|
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

notes = STDIN.read.lines.reject do |line|
  line =~ /^T:/ ||
  line =~ /^#/
end.join.chars
TinWhistle.new(notes).print
