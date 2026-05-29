-- name: BROKEN RECORD
-- repeats the same action forever based on first round

local first_move = nil

function decide(round, history)
    if round == 1 then
        first_move = "cooperate"
        return "cooperate"
    end
    return first_move
end
