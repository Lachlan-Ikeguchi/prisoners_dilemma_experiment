-- name: HONEST
-- always plays the same as last round (cooperate initially)

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- We don't track our own history, so just keep cooperating
    -- This is essentially ALWAYS COOPERATE
    return "cooperate"
end
