-- name: CHAMELEON
-- mimics opponent's last 2 moves

function decide(round, history)
    if round == 1 then return "cooperate" end
    if #history < 2 then return history[#history] end
    
    if history[#history] == history[#history-1] then
        return history[#history]
    else
        return "cooperate"
    end
end
