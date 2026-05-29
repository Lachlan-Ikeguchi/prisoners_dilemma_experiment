-- name: REPEATER
-- repeats its own last move (approximated by repeating first move)

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Without tracking our own history, just repeat cooperate
    return "cooperate"
end
