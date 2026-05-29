-- name: MIRROR
-- mirrors the opponent's last move

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    return history[#history]
end
