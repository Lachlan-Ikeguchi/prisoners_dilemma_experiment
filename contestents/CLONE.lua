-- name: CLONE
-- tries to replicate the opponent's last move

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Copy opponent's last move
    return history[#history]
end
