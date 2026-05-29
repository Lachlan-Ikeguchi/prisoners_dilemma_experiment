-- name: ECHO
-- repeats the opponent's last move from 2 rounds ago

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if #history >= 2 then
        return history[#history - 1]
    else
        return "cooperate"
    end
end
